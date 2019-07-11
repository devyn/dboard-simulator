use error_chain::error_chain;

error_chain! {
    foreign_links {
        WindowBuildError(sdl2::video::WindowBuildError);
        IntegerOrSdlError(sdl2::IntegerOrSdlError);
        TextureValueError(sdl2::render::TextureValueError);
    }
}
