// NVIC Description - is on pg 320
// NVIC Registers (Programming Manual) - is on 85

/* Nested vectored interrupt controller (NVIC) */

#[repr(C)]
pub struct NVICReg {
	pub iser:		[u32;  1],		/* Interrupt Set-Enable register */
	pub reserved0:	[u32; 31],
	pub icer:		[u32;  1], 		/* Interrupt Clear-Enable register */
	pub reserved1:	[u32; 31],
	pub ispr:		[u32;  1],    	/* Interrupt Set-Pending Registers */
	pub reserved2:	[u32; 31],
	pub icpr:		[u32;  1],	    /* Interrupt Clear-Pending Registers */
	pub reserved3:	[u32; 95],
	pub ipr:		[u8;  8],	    /* Interrupt Priority Registers */
}