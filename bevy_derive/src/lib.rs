extern crate proc_macro;

use darling::FromMeta;
use inflector::Inflector;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Field, Fields};

#[proc_macro_derive(EntityArchetype)]
pub fn derive_entity_archetype(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let fields = match &ast.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };

    let generics = ast.generics;
    let (impl_generics, ty_generics, _where_clause) = generics.split_for_impl();

    let struct_name = &ast.ident;
    let field_name = fields.iter().map(|field| &field.ident);

    TokenStream::from(quote! {
        impl #impl_generics bevy::prelude::EntityArchetype for #struct_name#ty_generics {
            fn insert(self, world: &mut bevy::prelude::World) -> Entity {
                *world.insert((), vec![(
                    #(self.#field_name,)*
                )]).first().unwrap()
            }
        }
    })
}
#[derive(FromMeta, Debug, Default)]
struct UniformAttributeArgs {
    #[darling(default)]
    pub ignore: Option<bool>,
    #[darling(default)]
    pub shader_def: Option<bool>,
}

#[proc_macro_derive(Uniforms, attributes(uniform))]
pub fn derive_uniforms(input: TokenStream) -> TokenStream {
    const UNIFORM_ATTRIBUTE_NAME: &'static str = "uniform";
    let ast = parse_macro_input!(input as DeriveInput);

    let fields = match &ast.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };

    let uniform_fields = fields
        .iter()
        .map(|f| {
            (
                f,
                f.attrs
                    .iter()
                    .find(|a| {
                        a.path.get_ident().as_ref().unwrap().to_string() == UNIFORM_ATTRIBUTE_NAME
                    })
                    .map(|a| {
                        UniformAttributeArgs::from_meta(&a.parse_meta().unwrap())
                            .unwrap_or_else(|_err| UniformAttributeArgs::default())
                    }),
            )
        })
        .collect::<Vec<(&Field, Option<UniformAttributeArgs>)>>();

    let active_uniform_fields = uniform_fields
        .iter()
        .filter(|(_field, attrs)| {
            attrs.is_none()
                || match attrs.as_ref().unwrap().ignore {
                    Some(ignore) => !ignore,
                    None => true,
                }
        })
        .map(|(f, _attr)| *f)
        .collect::<Vec<&Field>>();

    let shader_def_fields = uniform_fields
        .iter()
        .filter(|(_field, attrs)| match attrs {
            Some(attrs) => match attrs.shader_def {
                Some(shader_def) => shader_def,
                None => false,
            },
            None => false,
        })
        .map(|(f, _attr)| *f)
        .collect::<Vec<&Field>>();

    let shader_def_field_names = shader_def_fields.iter().map(|field| &field.ident);
    let shader_def_field_names_screaming_snake = shader_def_fields.iter().map(|field| {
        field
            .ident
            .as_ref()
            .unwrap()
            .to_string()
            .to_screaming_snake_case()
    });

    let struct_name = &ast.ident;
    let struct_name_screaming_snake = struct_name.to_string().to_screaming_snake_case();
    let field_infos_ident = format_ident!("{}_FIELD_INFO", struct_name_screaming_snake);

    let active_uniform_field_names = active_uniform_fields.iter().map(|field| {
        &field.ident
    }).collect::<Vec<_>>();

    let active_uniform_field_name_strings = active_uniform_fields.iter().map(|field| {
        field.ident.as_ref().unwrap().to_string()
    }).collect::<Vec<String>>();

    let mut uniform_name_strings = Vec::new();
    let mut texture_and_sampler_name_strings = Vec::new();
    let mut texture_and_sampler_name_idents = Vec::new();
    let field_infos = active_uniform_fields.iter().map(|f| {
        let field_name = f.ident.as_ref().unwrap().to_string();
        let uniform = format!("{}_{}", struct_name, field_name);
        let texture = format!("{}_texture", uniform);
        let sampler = format!("{}_sampler", uniform);
        uniform_name_strings.push(uniform.clone());
        texture_and_sampler_name_strings.push(texture.clone());
        texture_and_sampler_name_strings.push(sampler.clone());
        texture_and_sampler_name_idents.push(f.ident.clone());
        texture_and_sampler_name_idents.push(f.ident.clone());
        quote!(bevy::render::shader::FieldInfo {
            name: #field_name,
            uniform_name: #uniform,
            texture_name: #texture,
            sampler_name: #sampler,  
            is_vertex_buffer_member: false,
        })
    });

    TokenStream::from(quote! {
        const #field_infos_ident: &[bevy::render::shader::FieldInfo] = &[
            #(#field_infos,)*
        ];

        impl bevy::render::shader::AsUniforms for #struct_name {
            // TODO: max this an iterator that feeds on field_uniform_names_ident
            fn get_field_infos(&self) -> &[bevy::render::shader::FieldInfo] {
                #field_infos_ident
            }

            fn get_field_bind_type(&self, name: &str) -> Option<bevy::render::shader::FieldBindType> {
                use bevy::render::shader::AsFieldBindType;
                match name {
                    #(#active_uniform_field_name_strings => Some(self.#active_uniform_field_names.get_field_bind_type()),)*
                    _ => None,
                }
            }

            fn get_uniform_bytes(&self, name: &str) -> Option<Vec<u8>> {
                use bevy::core::bytes::GetBytes;
                match name {
                    #(#uniform_name_strings => Some(self.#active_uniform_field_names.get_bytes()),)*
                    _ => None,
                }
            }

            fn get_uniform_bytes_ref(&self, name: &str) -> Option<&[u8]> {
                use bevy::core::bytes::GetBytes;
                match name {
                    #(#uniform_name_strings => self.#active_uniform_field_names.get_bytes_ref(),)*
                    _ => None,
                }
            }

            fn get_uniform_texture(&self, name: &str) -> Option<bevy::asset::Handle<bevy::render::texture::Texture>> {
                use bevy::render::shader::GetTexture;
                match name {
                    #(#texture_and_sampler_name_strings => self.#texture_and_sampler_name_idents.get_texture(),)*
                    _ => None,
                }
            }

            // TODO: this will be very allocation heavy. find a way to either make this allocation free
            // or alternatively only run it when the shader_defs have changed
            fn get_shader_defs(&self) -> Option<Vec<String>> {
                use bevy::render::shader::ShaderDefSuffixProvider;
                let mut potential_shader_defs: Vec<(&'static str, Option<&'static str>)> = vec![
                    #((#shader_def_field_names_screaming_snake, self.#shader_def_field_names.get_shader_def()),)*
                ];

                Some(potential_shader_defs.drain(..)
                    .filter(|(f, shader_def)| shader_def.is_some())
                    .map(|(f, shader_def)| format!("{}_{}{}", #struct_name_screaming_snake, f, shader_def.unwrap()))
                    .collect::<Vec<String>>())
            }
        }
    })
}

#[proc_macro_derive(RegisterAppPlugin)]
pub fn derive_app_plugin(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let struct_name = &ast.ident;

    TokenStream::from(quote! {
        #[no_mangle]
        pub extern "C" fn _create_plugin() -> *mut bevy::plugin::AppPlugin {
            // TODO: without this the assembly does nothing. why is that the case?
            print!("");
            // make sure the constructor is the correct type.
            let object = #struct_name {};
            let boxed = Box::new(object);
            Box::into_raw(boxed)
        }
    })
}
