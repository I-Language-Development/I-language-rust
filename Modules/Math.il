/*
I Language math module.
Version: 1.0.0

Copyright (c) 2023-present I Language Development.

Permission is hereby granted, free of charge, to any person obtaining a
copy of this software and associated documentation files (the 'Software'),
to deal in the Software without restriction, including without limitation
the rights to use, copy, modify, merge, publish, distribute, sublicense,
and/or sell copies of the Software, and to permit persons to whom the
Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED 'AS IS', WITHOUT WARRANTY OF ANY KIND, EXPRESS
OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.

[doc]
	I Language math module.

	This module contains mathematical constants, functions and the complex type.
[end]
*/


////////////////////
// USE STATEMENTS //
////////////////////

use DualType;
use SelfType;


////////////////////////////
// MATHEMATICAL CONSTANTS //
////////////////////////////

// [doc]: π with 100 decimal places
const float Pi = 3.1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679;

// [doc]: e with 100 decimal places
const float e  = 2.7182818284590452353602874713526624977572470936999595749669676277240766303535475945713821785251664274;


//////////////////
// COMPLEX TYPE //
//////////////////

private class Complex(Type) {
	/*
	[doc]
		Complex number type.

		Represented like this:
		`1+2i`
	[end]
	*/

	function main(DualType(int, float) real, DualType(int. float) imaginary) {
		/*
		[doc]
			[args]
				real: Real part of the complex number.
				imaginary: Imaginary part of the complex number.
			[end]

			[example]
				complex(1, 2);
				// = 1+2i

				complex(1.2, 2.3);
				// = 1.2+2.3i

				complex(1.23456789, 2.34567890);
				// = 1.23+2.34
			[end]

			[warning]
				Float values get rounded to 2 decimal places.
				The values they were initialized with are stored in the [nobr]
				`COMPLEX.unrounded_real` and `COMPLEX.unrounded_imaginary` properties.
			[end]
		[end]
		*/


		if typeOf(real, float) {
			self.real = round(real, 2);
			self.unrounded_real = real;
		}
		else {
			self.real = real;
			self.unrounded_real = real; // Used to keep have access to this property at all times
		}

		if typeOf(imaginary, float) {
			self.imaginary = round(imaginary, 2);
			self.unrounded_imaginary = imaginary;
		}
		else {
			self.imaginary = imaginary;
			self.unrounded_imaginary = imaginary;  // Used to keep have access to this property at all times
		}

		parent.main(self.real, self.imaginary);
	}

	string repr() {
		return self.real + "+" + self.imaginary + "i";
	}

	Self add(Self other) {
		return Self(self.real + other.real, self.imaginary + other.imaginary);
	}
}

export Complex as complex;
