pub mod mono_stream;
pub mod stereo_stream;
pub mod consts;

pub mod prelude {
    #[doc(hidden)]
    pub use crate::mono_stream::MonoStream;
    #[doc(hidden)]
    pub use crate::stereo_stream::StereoStream;
    #[doc(hidden)]
    pub use crate::consts::AudioConsts;
}