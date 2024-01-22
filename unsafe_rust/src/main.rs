use std::slice;

extern "C" {
    fn abs(input: i32) -> i32;
}

static HELLO_WORLD: &str = "Hello, world!"; //must have static lifetime

fn main() {
    
    let mut num = 5;
    let r1 = &num as *const i32; //immutable raw pointer
    let r2 = &mut num as *mut i32; //mutable raw pointer

    //1. Raw pointers ignore rust borrowing -> multiple immut and mut pointers allowed
    //2. Raw pointers can be null
    //3. Dont implement automatic clean up
    //4. Cant deref raw pointers unless in an unsafe block

    unsafe {
        println!("Here is r1 {}",*r1);
        println!("HERE is r2 {}",*r2);
        //can have mutable and immutable pointer to same value 
    }
    let address = 0x012345usize;
    let r3 = address as *const i32; //not sure if we have valid memory or not   

    unsafe fn dangerous() {
        //when calling function need to give correct arguments otherwise it returns errors
    }

    unsafe{
        dangerous();
    }
    //can be called in other unsafe functions or an unsafe block

    let mut v = vec![1,2,3,4,5,6];

    let r = &mut v[..];
    let (a,b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1,2,3]);
    assert_eq!(b, &mut [4,5,6]);
    let mut v2 = vec![1,2,3,4,5,6];
    let r2 = &mut v2[..];
    let (c,d) = split_at_mut2(r2,3);


    unsafe{
        println!("Absolute value of -3 according to C is {}",abs(3)); 
        //unsafe for foreign language interface
    }

    println!("name is {}",HELLO_WORLD);
}

fn split_at_mut2(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {

    let len = slice.len();
    let ptr = slice.as_mut_ptr(); //getting raw mutable pointer from slice
    //this raw mutable pointer helps us to work in unsafe code and use the same pointer multiple times
    //this is not happening in the function below hence causing an error
    assert!(mid<=len);

    unsafe{
        (
            slice::from_raw_parts_mut(ptr, mid), //unsafe because it only works for valid values
            slice::from_raw_parts_mut(ptr.add(mid),len-mid), //unsafe because mid must be valid value
        )
    } //create safe abstraction under unsafe code so the function itself is safe
}

/*unsafe fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();

    assert!(mid<=len);


    (&mut slice[..mid], &mut slice[mid..]) //
    //code wont compile because we are referencing the same slice mutably twice
    //not acceptable by rust even though it makes sense logically
}*/

