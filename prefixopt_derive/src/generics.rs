use syn::*;

pub fn with_prefixopt_constraints(mut generics: Generics) -> Generics {
    //panic!("{:?}", generics.where_clause.predicates);
    for ty_param in generics.ty_params.iter_mut() {
        let prefix_ty_bound = {
            let path = Path::from("PrefixOpt");
            let trait_ref = PolyTraitRef {
                bound_lifetimes: vec![],
                trait_ref: path,
            };
            TyParamBound::Trait(trait_ref, TraitBoundModifier::None)
        };
        let where_bound = identity_where_bound(&ty_param.ident);
        ty_param.bounds.push(prefix_ty_bound);
        generics.where_clause.predicates.push(where_bound);
    }
    generics
}
pub fn with_default_constraints(mut generics: Generics) -> Generics {
    for ty_param in generics.ty_params.iter_mut() {
        let default_ty_bound = {
            let path = Path::from("Default");
            let trait_ref = PolyTraitRef {
                bound_lifetimes: vec![],
                trait_ref: path,
            };
            TyParamBound::Trait(trait_ref, TraitBoundModifier::None)
        };
        ty_param.bounds.push(default_ty_bound);
    }
    generics
}

/** Creates a where bound of the following form:
* <#ty_param_ident as PrefixOpt>::Container: PrefixOptContainer<Parsed = #ty_param_ident>
*/
fn identity_where_bound(ty_param_ident: &Ident) -> WherePredicate {
    let ty_path = Path {
        global: false,
        segments: vec![PathSegment::from("PrefixOpt"),
                       PathSegment::from("Container")],
    };
    let orig_type_path = Ty::Path(None, Path::from(ty_param_ident.clone()));
    let ty = Ty::Path(Some(QSelf {
                               position: 1,
                               ty: Box::new(orig_type_path.clone()),
                           }),
                      ty_path);
    let parsed_is_generic = AngleBracketedParameterData {
        lifetimes: vec![],
        types: vec![],
        bindings: vec![TypeBinding {
                           ident: Ident::from("Parsed"),
                           ty: orig_type_path,
                       }],
    };
    let trait_ref = PolyTraitRef {
        bound_lifetimes: vec![],
        trait_ref: Path::from(PathSegment {
                                  ident: Ident::from("PrefixOptContainer"),
                                  parameters: PathParameters::AngleBracketed(parsed_is_generic),
                              }),
    };
    let bound = WhereBoundPredicate {
        bound_lifetimes: vec![],
        bounded_ty: ty,
        bounds: vec![TyParamBound::Trait(trait_ref, TraitBoundModifier::None)],
    };
    WherePredicate::BoundPredicate(bound)
}

pub fn filter_types<'a, I>(gen: &Generics, types: I) -> Generics
    where I: Iterator<Item = &'a Ty>
{
    use std::collections::HashSet;

    fn handle_path<'a>(p: &'a Path, explore: &mut Vec<&'a Ty>, unused: &mut HashSet<&Ident>) {
        for seg in p.segments.iter() {
            segment_explore_generics(seg, explore);
        }
        if !p.global {
            unused.remove(&p.segments.first().unwrap().ident);
        }
    }
    fn segment_explore_generics<'a>(seg: &'a PathSegment, explore: &mut Vec<&'a Ty>) {
        match seg.parameters {
            PathParameters::AngleBracketed(ref angle) => {
                explore.extend(angle.types.iter());
                explore.extend(angle.bindings.iter().map(|binding| &binding.ty))
            }
            PathParameters::Parenthesized(ref paren) => {
                explore.extend(paren.inputs.iter());
                if let Some(ref out) = paren.output.as_ref() {
                    explore.push(&out)
                }
            }
        }
    }
    fn get_path(ty_param: &TyParamBound) -> Option<&Path> {
        match *ty_param {
            TyParamBound::Region(_) => None,
            TyParamBound::Trait(ref p, _) => Some(&p.trait_ref),
        }
    }
    let mut unused = gen.ty_params
        .iter()
        .map(|ty| &ty.ident)
        .collect::<HashSet<_>>();
    let mut explore = types.collect::<Vec<_>>();
    while let Some(ty) = explore.pop() {
        use Ty::*;
        match *ty {
            Paren(ref ty) | Slice(ref ty) | Array(ref ty, _) => explore.push(&ty),
            Ptr(ref mutty) |
            Rptr(_, ref mutty) => explore.push(&mutty.ty),
            BareFn(ref barefn) => {
                explore.extend(barefn.inputs.iter().map(|arg| &arg.ty));
                match barefn.output {
                    FunctionRetTy::Ty(ref ty) => explore.push(ty),
                    FunctionRetTy::Default => (),
                };
            }

            TraitObject(ref ty_params) |
            ImplTrait(ref ty_params) => {
                let paths = ty_params.iter().filter_map(&get_path);
                for p in paths {
                    handle_path(p, &mut explore, &mut unused);
                }
            }
            Path(ref qself, ref p) => {
                if let Some(ref qself) = qself.as_ref() {
                    explore.push(&qself.ty);
                }
                handle_path(&p, &mut explore, &mut unused)
            },
            Tup(ref tys) => explore.extend(tys),
            Never | Infer | Mac(_) => (),
        }
    }

    panic!()

}

pub fn phantoms<'a>(gen: &'a Generics) -> Box<Iterator<Item = (&'a Ident, Ty)> + 'a> {
    let idents = gen.ty_params.iter().map(|ty_param| &ty_param.ident);
    let phantom_pair = idents.map(|id| (id, as_phantomdata_type(id)));
    Box::new(phantom_pair)
}

fn as_phantomdata_type(id: &Ident) -> Ty {
    let as_path = Path::from(id.clone());
    let as_type = Ty::Path(None, as_path);
    let angle_bracket = PathParameters::AngleBracketed(AngleBracketedParameterData {
                                                           lifetimes: vec![],
                                                           types: vec![as_type],
                                                           bindings: vec![],
                                                       });
    let phantom_segment = PathSegment {
        ident: Ident::from("PhantomData"),
        parameters: angle_bracket,
    };
    let fully_qualified_path = Path {
        global: true,
        segments: vec![From::from("std"), From::from("marker"), phantom_segment],
    };
    Ty::Path(None, fully_qualified_path)
}
