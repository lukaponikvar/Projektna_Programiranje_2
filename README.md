# Project for class Programiranje 2

## Server functionality

This project is an implementation of a server that can 
communicate with other servers logged in to the main server. 
The desired result is uninterupted exchange of sequence 
information between various servers, each possessing disparate sequences.

It can handle the following `GET` requests:
- `/ping`: responds with the information about the server including: `name` and `address`,
- `/sequence`: provides the information about available sequences,

and the following `POST` request:
- `/sequence`: returns a vector of floating point numbers, 
according to the specified sequence and range.

## Running the Servers

Starting the server requires use of the following command in the
 terminal: 
 `cargo run -- register_address generator_ip generator_port`,
 where the defaults are `127.0.0.1:7878`, `127.0.0.1` and `9000` respectively.

> [!WARNING]
> Make sure that the main server is up and running
 before starting, so as to make sure, that this server logs in to the
 main server.

In the `machine\tests` folder there is an existing `test.py` 
file with pre-written tests for each available sequence, that
 can be modified, should one request a sequence located on 
 another server.

## Available sequences

> __Arithmetic sequence__  
> _Description:_ 
> Arithmetic sequence which takes two parameters: a start and a 
step.  
> _Parameters:_ 
>  2  
> _Sequences:_ 
>  0

> __Average sequence__  
> _Description:_ 
> A sequence that takes two sequences and calculates their 
average term by term.  
> _Parameters:_ 
>  0  
> _Sequences:_ 
>  2

> __Constant sequence__  
> _Description:_ 
> Constant sequence with a single parameter: a value.  
> _Parameters:_ 
>  1  
> _Sequences:_ 
>  0

> __Drop sequence__  
> _Description:_ 
> A sequence which takes a sequence and a shift parameter and is 
> equivalent to shifting that particular sequence by the given
 amount.  
> _Parameters:_ 
> 1  
> _Sequences:_ 
> 1

> __Fibonacci sequence__  
> _Description:_ 
> Fibonacci sequence starting with given parameters `zeroth` and
 `first`.  
> _Parameters:_ 
>  2  
> _Sequences:_ 
>  0

> __Geometric sequence__  
> _Description:_ 
> Geometric sequence with two parameters: a start and a 
quotient.  
> _Parameters:_ 
>  2  
> _Sequences:_ 
>  0

> __Linear Combination sequence__  
> _Description:_ 
> Linear combination of two sequences `a` and `b` as given by:
 `(x*a_i + y*b_i + w)_i` where `x`, `y` and `w` are the three 
 parameters.  
> _Parameters:_ 
>  3  
> _Sequences:_ 
>  2

> __Max sequence__  
> _Description:_ 
> A sequence that takes two sequences and uses their maximum 
term by term.  
> _Parameters:_ 
>  0  
> _Sequences:_ 
>  2

> __Min sequence__  
> _Description:_ 
> A sequence that takes two sequences and uses their minimum 
term by term.  
> _Parameters:_ 
>  0  
> _Sequences:_ 
>  2

> __Product sequence__  
> _Description:_ 
> A sequence that takes two sequences and multiplies them term 
by term.  
> _Parameters:_ 
>  0  
> _Sequences:_ 
>  2

> __Sum sequence__  
> _Description:_ 
> A sequence that takes two sequences and adds them term by term.  
> _Parameters:_ 
>  0  
> _Sequences:_ 
>  2

> [!NOTE]
> All sequences that could logically take an arbitrary amount of sequences are implemented in such a way, however the project restrictions make it unfeasible.

### Authors:
- Anara Nemanič
- Luka Ponikvar

### Mentors:
- asist. Filip Koprivec
- doc. dr. Matija Pretnar

