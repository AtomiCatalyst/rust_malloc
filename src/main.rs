mod errors;
mod node;
use std::{fs::File, mem::size_of, usize};
use page_size;
use memmap::{MmapMut, MmapOptions};
use errors::Error::ErrorType as ErrorType;
use node::node::Node_t as Node_t;

static MAX_ARENA_SIZE: usize = 0x7FFFFFFF;

fn main() {

    let resu = RustMalloc::init(32768).unwrap();
    println!("Success! {:?} {:?}", resu.arena_ptr, resu.free_list);
    resu.destroy();
}

#[derive(Debug)]
struct RustMalloc<'a> {
    arena_ptr: *mut u8,
    free_list: Vec<Node_t<'a>>
}

impl RustMalloc<'_> {
    fn init<'a>(size: usize) -> Result<RustMalloc<'a>, ErrorType> {
        let page_s: usize = page_size::get();
        let fd: Option<File> = File::options()
            .read(true)
            .write(true)
            .open("/dev/zero")
            .ok(); 
    
        if fd.is_none() {
            return Err(ErrorType::ERR_SYSCALL_FAILED);
        }
    
        println!("Initializing arena:");
        println!("...requested size {} bytes", size);
        if size > MAX_ARENA_SIZE || size <= 0 {
           return Err(ErrorType::ERR_BAD_ARGUMENTS);
        }
    
        println!("...pagesize is {} bytes", page_s);
        println!("...adjusting size with page boundaries");
    
        let mut scale = 0;
        let mut temp = size;
        loop {
            if temp <= 0 {
                break;
            }
            temp -= page_s;
            scale += 1;
        }
    
        
        let _arena_size: usize = scale * page_s;
    
        if _arena_size > MAX_ARENA_SIZE {
            println!("...error: requested size larger than MAX_ARENA_SIZE: {}", MAX_ARENA_SIZE);
            return Err(ErrorType::ERR_BAD_ARGUMENTS)
        }
    
        println!("...adjusting size is {} bytes", _arena_size);
        println!("...mapping arena with memmap");
    
        let _arena_start: Option<MmapMut> = unsafe { 
            MmapOptions::new()
                .len(_arena_size)
                .map_mut(&fd
                        .as_ref()
                        .unwrap())
                .ok()
            };
    
        if _arena_start.is_none() {
            println!("...failed in initialization, memmap");
            return Err(ErrorType::ERR_SYSCALL_FAILED);
        }
    
        let _arena_start_copy_print = _arena_start.as_ref().unwrap().as_ptr();
        let _arena_start_copy: usize = _arena_start_copy_print as usize;
    
        println!("...arena starts at {:#?} + {}", _arena_start_copy_print, size_of::<Node_t>());
        println!("...intializing header for initial free chunk");
    
        let head: Node_t = Node_t {
            size: (_arena_size - size_of::<Node_t>()),
            is_free: true,
            fwd: None,
            bwd: None
        };
    
        println!("...header size is {} bytes", size_of::<Node_t>());

        let malloc = RustMalloc {
            arena_ptr: _arena_start.unwrap().as_mut_ptr(),
            free_list: vec![head]
        };
    
        
        Ok(malloc)
    }

    fn destroy(self) {
        drop(self.arena_ptr) //calls into function and then drops itself
    }

}



