import grpc from "@grpc/grpc-js"
import protoLoader from "@grpc/proto-loader"
// this is the path to the proto idl
const path="../data.proto";
// we create a package definiton object
// =========================================================================================================
// keepCase: true → keeps field names exactly as in the .proto file.

// longs: String → converts 64-bit integers to JS strings.

// enums: String → converts enum values to strings.

// defaults: true → fills in missing fields with default values.

// oneofs: true → tracks which field of a oneof group is set.
const packageDefinition=protoLoader.loadSync(path,{
    keepCase:true,
    defaults:true,
    enums:String,
    longs: String,
    oneofs: true
})
// ====================================================================================================================
// grpc.loadPackageDefinition() converts the package definition into actual gRPC objects.

// .helloworld → accesses the grpc_dummy package from the .proto file.
const grpc_dummy=grpc.loadPackageDefinition(packageDefinition).grpc_dummy;

// now we define the function that correspond to the method defined inside of the proto's service Details expression

function get_details(call,callback)
{
//     Defines the function that handles the SayHello RPC.

// call → contains the request info (e.g., call.request.name).

// callback(error, response) → used to send the response back to the client.

// In this example, it responds with a message: "Hello <name>".
 
//  access the data send from the call request
const {email}=call.request;

callback(null,{
    name:"jason",
    email,
    age:23
})
}


function main()
{
    const server=new grpc.Server();
    server.addService(grpc_dummy.Details.service,{"get_detail":get_details});
    // ServerCredential tells how the raw binary format should be shared over meaning over the sream
    // right now we are using the http2 without any sort of encryption
    server.bindAsync('0.0.0.0:50051', grpc.ServerCredentials.createInsecure(), (err, port) => {
        if (err) {
          console.error('Server binding failed:', err);
          return;
        }
        console.log(`Server is running on port ${port}`);
        // No need to call server.start()
      });
}

main()