//! SuperLU performance tuning
//!
//! This module exposes the performance-tuning parameters of SuperLU,
//! which allow the user to specify parameters that relate closely to
//! the computer architecture or the types of systems being solved.
//!
//! This function has been moved out of the superlu C code in order to
//! allow access by rust functions.

#[derive(Copy, Clone)]
pub struct TuningParams{
    pub panel_size: usize,
    /// When the elimination tree is constructed,
    /// there may be supernodes of very small size.
    /// This are grouped together (relaxation) into
    /// artificial supernodes that are larger, but
    /// contain some zeros. This parameter sets the
    /// cutoff at which this grouping will occur,
    /// which sets the minimum size of the supernodes.
    pub relaxation_param: usize,
    pub max_supernode_size: usize,
    pub min_row_2d_block: usize,
    pub min_col_2d_block: usize,
    pub estimated_fills: usize,
    pub max_ilu_supernode_size: usize,
}

impl TuningParams {
    const fn new() -> Self {
	// TODO: check these match the superlu defaults
	Self {
	    panel_size: 20,
	    relaxation_param: 10,
	    max_supernode_size: 200,
	    min_row_2d_block: 200,
	    min_col_2d_block: 100,
	    estimated_fills: 30,
	    max_ilu_supernode_size: 10,
	}
    }

    fn sp_ienv(&self, ispec: libc::c_int) -> libc::c_int {
	(match ispec {
	    1 => self.panel_size,
	    2 => self.relaxation_param,
	    3 => self.max_supernode_size,
	    4 => self.min_row_2d_block,
	    5 => self.min_col_2d_block,
            6 => self.estimated_fills,
            7 => self.max_ilu_supernode_size,
	    _ => panic!("Invalid ispec in (rust) sp_ienv")
	}) as libc::c_int
    }
}

static mut tuning_params: TuningParams = TuningParams::new();

pub unsafe fn get_tuning_params() -> TuningParams {
    tuning_params
}

pub unsafe fn set_tuning_params(new_params: TuningParams) {
    tuning_params = new_params;
}


/// Return performance-tuning parameters to the SuperLU library routines
///
/// The sp_ienv C function in superlu-5.3.0/SRC/sp_ienv.c is
/// commented out, and is replaced by this function when the csuperlu_sys
/// library is linked. Below is the documentation copied directly from the
/// superlu source.
///
/// Note that the example values used for these parameters are different
/// for the examples. To make the examples in the superlu user guide work,
/// see sp_ienv in superlu-5.3.0/EXAMPLES/
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
    unsafe {
	tuning_params.sp_ienv(ispec)
    }
}
