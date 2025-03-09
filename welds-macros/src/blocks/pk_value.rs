use crate::info::Info;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn write(info: &Info) -> TokenStream {
    let defstruct = &info.defstruct;
    let pks = info.pks.as_slice();
    let wp = &info.welds_path;
    if pks.is_empty() {
        return quote!();
    }

    let pk_types: Vec<_> = pks.iter().map(|p| &p.field_type).collect();
    let pk_names: Vec<_> = pks.iter().map(|p| &p.field).collect();
    let pk_value = quote! { (#(#pk_types),*) };

    quote! {

        impl #wp::model_traits::PkValue for #defstruct {
            type PkVal = #pk_value;

            fn pk_value(&self) -> Self::PkVal
            where
                <Self as #wp::model_traits::HasSchema>::Schema: #wp::model_traits::TableColumns,
            {
                (#(self.#pk_names.clone()),*)
            }
        }

    }
}
