const express = require("express")
const app = express()
const { graphqlHTTP } = require("express-graphql")
const { buildSchema } = require("graphql")
let clientes =  new Array()
let contador = 1

var schema = buildSchema(`
type Cliente {
    id: Int
    nombre: String
    telefono: String
}
type Query {
    clientes: [Cliente]
    cliente(id: Int): Cliente
}

type Mutation {
    addCliente(nombre: String, telefono: String): Cliente   
}
`);

var root = {
    clientes: () => { return clientes },
    cliente: (data) => {
        clientes.forEach(client => {
            if (client.id == data.id) return client;
        });
    },
    addCliente: (data) => {
        var newClient = {
            "id": contador,
            "nombre": data.nombre,
            "telefono": data.telefono,
        };
        clientes.push(newClient);
        contador++;
        return newClient;
    }

}

app.use("/graphql", graphqlHTTP({
    schema: schema,
    rootValue: root,
    graphiql: true,
}))

app.listen(4000, () => { console.log("Running graphql server on port 4000"); })
