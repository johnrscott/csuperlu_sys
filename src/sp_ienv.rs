
/// Return performance-tuning parameters to the SuperLU library routines
///
/// The sp_ienv C function in superlu-5.3.0/SRC/sp_ienv.c is
/// commented out, and is replaced by this function when the csuperlu_sys
/// library is linked. Below is the documentation copied directly from the
/// superlu source.
///
/// # Purpose   
///
/// sp_ienv() is inquired to choose machine-dependent parameters for the
/// local environment. See ISPEC for a description of the parameters.   
///
/// This version provides a set of parameters which should give good,   
/// but not optimal, performance on many of the currently available   
/// computers.  Users are encouraged to modify this subroutine to set   
/// the tuning parameters for their particular machine using the option   
/// and problem size information in the arguments.   
///
/// # Arguments   
///
/// ISPEC   (input) int
///         Specifies the parameter to be returned as the value of SP_IENV.   
///         = 1: the panel size w; a panel consists of w consecutive
///	         columns of matrix A in the process of Gaussian elimination.
///		 The best value depends on machine's cache characters.
///         = 2: the relaxation parameter relax; if the number of
///	         nodes (columns) in a subtree of the elimination tree is less
///		 than relax, this subtree is considered as one supernode,
///		 regardless of their row structures.
///         = 3: the maximum size for a supernode in complete LU;
///	    = 4: the minimum row dimension for 2-D blocking to be used;
///	    = 5: the minimum column dimension for 2-D blocking to be used;
///	    = 6: the estimated fills factor for L and U, compared with A;
///	    = 7: the maximum size for a supernode in ILU.
///	    
/// (SP_IENV) (output) int
///         >= 0: the value of the parameter specified by ISPEC   
///         < 0:  if SP_IENV = -k, the k-th argument had an illegal value. 
#[no_mangle]
pub extern "C" fn sp_ienv(ispec: libc::c_int) -> libc::c_int {
    println!("Hello from conterfeit sp_ienv!");
    match ispec {
	1 => 20,
	2 => 10,
	3 => 200,
	4 => 200,
	5 => 100,
        6 => 30,
        7 => 10,
	_ => panic!("Invalid ispec in (rust) sp_ienv")
    }
}
