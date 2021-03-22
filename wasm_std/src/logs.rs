//! 预留debug处理

mod external{
    extern {
        pub fn debug(msg_ptr: *const u8, msg_len: u32);
    }
}

/// log of runtime's debug mode
pub fn debug<'a>(msg: &'a str){
    unsafe{
        external::debug(msg.as_ptr(),msg.len() as u32);
    }
}

