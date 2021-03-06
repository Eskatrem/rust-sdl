use std::cast;
use std::libc::c_int;
use std::str;

use get_error;

pub mod ll {
	use std::libc::{c_int, c_schar, uint8_t, uint16_t, uint32_t};

	pub type CDstatus = c_int;

	pub static CD_TRAYEMPTY: CDstatus = 0;
	pub static CD_STOPPED: CDstatus = 1;
	pub static CD_PLAYING: CDstatus = 2;
	pub static CD_PAUSED: CDstatus = 3;
	pub static CD_ERROR: CDstatus = -1;

	pub struct SDL_CDtrack {
	    id: uint8_t,
	    _type: uint8_t,
	    unused: uint16_t,
	    length: uint32_t,
	    offset: uint32_t
	}

	pub struct SDL_CD {
	    id: c_int,
	    status: CDstatus,
	    numtracks: c_int,
	    cur_track: c_int,
	    cur_frame: c_int,
	    track: [SDL_CDtrack, ..100],
	}

    externfn!(fn SDL_CDNumDrives() -> c_int)
    externfn!(fn SDL_CDName(drive: c_int) -> *c_schar)
    externfn!(fn SDL_CDOpen(drive: c_int) -> *SDL_CD)
    externfn!(fn SDL_CDStatus(cdrom: *SDL_CD) -> CDstatus)
    externfn!(fn SDL_CDClose(cdrom: *SDL_CD))
    externfn!(fn SDL_CDStop(cdrom: *SDL_CD) -> c_int)
    externfn!(fn SDL_CDEject(cdrom: *SDL_CD) -> c_int)
    externfn!(fn SDL_CDResume(cdrom: *SDL_CD) -> c_int)
    externfn!(fn SDL_CDPlay(cdrom: *SDL_CD, start: c_int, length: c_int) -> c_int)
    externfn!(fn SDL_CDPlayTracks(cdrom: *SDL_CD,
                                  start_track: c_int,
                                  start_frame: c_int,
                                  ntracks: c_int,
                                  nframes: c_int) -> c_int)
    externfn!(fn SDL_CDPause(cdrom: *SDL_CD) -> c_int)
}

pub fn get_num_drives() -> int {
	unsafe { ll::SDL_CDNumDrives() as int }
}

pub fn get_drive_name(index: int) -> ~str {
	unsafe {
		let cstr = ll::SDL_CDName(index as c_int);

		str::raw::from_c_str(cast::transmute_copy(&cstr))
	}
}

#[deriving(Eq)]
pub struct CD {
	raw: *ll::SDL_CD
}

fn wrap_cd(raw: *ll::SDL_CD) -> ~CD {
	~CD { raw: raw }
}

#[deriving(Eq)]
pub enum Status {
	TrayEmptyStatus = ll::CD_TRAYEMPTY as int,
	StoppedStatus = ll::CD_STOPPED as int,
	PlayingStatus = ll::CD_PLAYING as int,
	PausedStatus = ll::CD_PAUSED as int,
	ErrorStatus = ll::CD_ERROR as int
}

impl CD {
    pub fn open(index: int) -> Result<~CD, ~str> {
		unsafe {
			let raw = ll::SDL_CDOpen(index as c_int);

			if raw.is_null() { Err(get_error()) }
			else { Ok(wrap_cd(raw)) }
		}
	}

	pub fn get_status(&self) -> Status {
		unsafe {
			// FIXME: Rust doesn't like us matching using staticants here for some reason
			match ll::SDL_CDStatus(self.raw) {
				0 => TrayEmptyStatus,
				1 => StoppedStatus,
				2 => PlayingStatus,
				3 => PausedStatus,
				-1 => ErrorStatus,
				_ => ErrorStatus
			}
		}
	}

	pub fn play(&self, start: int, len: int) -> bool {
		unsafe { ll::SDL_CDPlay(self.raw, start as c_int, len as c_int) == 0 }
	}

	pub fn play_tracks(&self, start_track: int, start_frame: int, ntracks: int,
		           nframes: int) -> bool {
		unsafe {
			ll::SDL_CDPlayTracks(self.raw, start_track as c_int, start_frame as c_int,
				                 ntracks as c_int, nframes as c_int) == 0
		}
	}

	pub fn pause(&self) -> bool {
		unsafe { ll::SDL_CDPause(self.raw) == 0 }
	}

	pub fn resume(&self) -> bool {
		unsafe { ll::SDL_CDResume(self.raw) == 0 }
	}

	pub fn stop(&self) -> bool {
		unsafe { ll::SDL_CDStop(self.raw) == 0 }
	}
}

impl Drop for CD {
    fn drop(&self) {
        unsafe { ll::SDL_CDClose(self.raw); }
    }
}
