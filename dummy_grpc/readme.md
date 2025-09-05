# google remote procedural call

### it uses protocol buffer 

## Protocol Buffer

* Protocol Buffers are a language-neutral, platform-neutral extensible mechanism for serializing structured data.

example
  
  Person
  {
      age:23,
      name:"rohan"
  }

-> this type of structured data

* It’s like JSON, except it’s smaller and faster, and it generates native language bindings.

* we define how data has to be structured 

* we can then send it over the any stream

* this can be then received ont he other end
ehich can be used to construct the data

* Protocol buffers are a combination of the definition language (created in .proto files),

* the compiler of the protocol buffer genrates the
code in the language we are targetting ,languages of the endpoint

* this code is the interface to interact with 
coming data