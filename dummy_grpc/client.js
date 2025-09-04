import grpc from "@grpc/grpc-js"
import protoLoader from "@grpc/proto-loader"
// this is the path to the proto idl
const path="../data.proto";


const packageDefinition=protoLoader.loadSync(path,
    {
        keepCase:true,
    defaults:true,
    enums:String,
    longs: String,
    oneofs: true
    }
)

// now load the package
const grpc_dummy=grpc.loadPackageDefinition(packageDefinition).grpc_dummy;

function main()
{
    const client=new grpc_dummy.Details("localhost:50051", grpc.credentials.createInsecure());
    client.get_detail({email:"my_email@gmail.com"},function(err,response)
{
   console.log(response);
}
)
}
main()