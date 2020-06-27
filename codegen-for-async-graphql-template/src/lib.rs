mod base;
mod template;

pub use base::{
    generate_from_path, generate_token_from_path, Config, Context, FileRender, RenderType,
    RendererFieldType, RendererInterfaceType, RendererObjectType, RendererScalarType, SupportField,
};

pub use template::utils::snake_case;
