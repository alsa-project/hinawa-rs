use glib::translate::*;

use SndMotuRegisterDspParameter;

impl SndMotuRegisterDspParameter {
    pub fn get_input_flag(&self) -> &[u8; 10] {
        unsafe {
            let mut ptr = std::ptr::null_mut() as *const [u8; 10];
            hinawa_sys::hinawa_snd_motu_register_dsp_parameter_get_input_flag(
                self.to_glib_none().0,
                &mut ptr,
            );
            &*ptr
        }
    }

    pub fn get_input_gain_and_invert(&self) -> &[u8; 10] {
        unsafe {
            let mut ptr = std::ptr::null_mut() as *const [u8; 10];
            hinawa_sys::hinawa_snd_motu_register_dsp_parameter_get_input_gain_and_invert(
                self.to_glib_none().0,
                &mut ptr,
            );
            &*ptr
        }
    }

    pub fn get_mixer_output_paired_flag(&self) -> &[u8; 4] {
        unsafe {
            let mut ptr = std::ptr::null_mut() as *const [u8; 4];
            hinawa_sys::hinawa_snd_motu_register_dsp_parameter_get_mixer_output_paired_flag(
                self.to_glib_none().0,
                &mut ptr,
            );
            &*ptr
        }
    }

    pub fn get_mixer_output_paired_volume(&self) -> &[u8; 4] {
        unsafe {
            let mut ptr = std::ptr::null_mut() as *const [u8; 4];
            hinawa_sys::hinawa_snd_motu_register_dsp_parameter_get_mixer_output_paired_volume(
                self.to_glib_none().0,
                &mut ptr,
            );
            &*ptr
        }
    }

    pub fn get_mixer_source_flag(&self, mixer: usize) -> &[u8; 20] {
        unsafe {
            let mut ptr = std::ptr::null_mut() as *const [u8; 20];
            hinawa_sys::hinawa_snd_motu_register_dsp_parameter_get_mixer_source_flag(
                self.to_glib_none().0,
                mixer,
                &mut ptr,
            );
            &*ptr
        }
    }

    pub fn get_mixer_source_gain(&self, mixer: usize) -> &[u8; 20] {
        unsafe {
            let mut ptr = std::ptr::null_mut() as *const [u8; 20];
            hinawa_sys::hinawa_snd_motu_register_dsp_parameter_get_mixer_source_gain(
                self.to_glib_none().0,
                mixer,
                &mut ptr,
            );
            &*ptr
        }
    }

    pub fn get_mixer_source_paired_balance(&self, mixer: usize) -> &[u8; 20] {
        unsafe {
            let mut ptr = std::ptr::null_mut() as *const [u8; 20];
            hinawa_sys::hinawa_snd_motu_register_dsp_parameter_get_mixer_source_paired_balance(
                self.to_glib_none().0,
                mixer,
                &mut ptr,
            );
            &*ptr
        }
    }

    pub fn get_mixer_source_paired_width(&self, mixer: usize) -> &[u8; 20] {
        unsafe {
            let mut ptr = std::ptr::null_mut() as *const [u8; 20];
            hinawa_sys::hinawa_snd_motu_register_dsp_parameter_get_mixer_source_paired_width(
                self.to_glib_none().0,
                mixer,
                &mut ptr,
            );
            &*ptr
        }
    }

    pub fn get_mixer_source_pan(&self, mixer: usize) -> &[u8; 20]{
        unsafe {
            let mut ptr = std::ptr::null_mut() as *const [u8; 20];
            hinawa_sys::hinawa_snd_motu_register_dsp_parameter_get_mixer_source_pan(
                self.to_glib_none().0,
                mixer,
                &mut ptr,
            );
            &*ptr
        }
    }
}
