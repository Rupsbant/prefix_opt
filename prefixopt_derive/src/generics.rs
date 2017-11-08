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
