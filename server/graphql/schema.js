const { buildSchema } = require("graphql")

module.exports = buildSchema(`
  type User {
    _id: ID!
    email: String!
    logged: Boolean!
  }

  type Query {
    users: [User!]
  }

  schema {
    query: Query
  }
`)