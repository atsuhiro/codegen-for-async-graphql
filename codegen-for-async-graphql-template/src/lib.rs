mod base;
mod template;

pub use base::{
    generate_from_path, generate_token_from_path, Config, Context, FileRender, RenderType,
    RendererFieldType, RendererInterfaceType, RendererMutationType, RendererMutationsType,
    RendererObjectType, RendererScalarType, SupportField, SupportType,
};

pub use template::utils::snake_case;
