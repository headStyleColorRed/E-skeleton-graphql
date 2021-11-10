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

  input UserInput {
    email: String!
  }

  type Mutation {
    createUser(user: UserInput): User
  }

  schema {
    query: Query
    mutation: Mutation
  }
`)