mod extension;

use super::{Context, RenderType, RendererScalarType, Save};

use extension::Renderer;

pub fn generate_scalar_type_file(context: &mut Context) {
    context.clone().scalar_types().iter().for_each(|f| {
        Renderer::model_file(f, context);
    });
}
