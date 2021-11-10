const express = require("express")
const app = express();
const port = 4000;
const bodyParser = require('body-parser');
const Cors = require("cors")
const mongoose = require("mongoose")
const environment = process.env.NODE_ENV
const jwt = require('jsonwebtoken');
var dbLink = new String()
require('dotenv').config()

// Modules
const User = require("./mongoDB/userModel.js")

// Set environment
console.log(`Current environment -> ${environment}`);
if (environment == "production")
  dbLink = "mongodb://graphql_DB:27017/mongologin"
else
  dbLink = "mongodb://localhost:27017/mongologin"


// Middlewares
app.use(Cors());
app.use(bodyParser.json());
app.use(bodyParser.urlencoded({ extended: false }))

// Open port
app.listen(port, () => console.log("Listening port " + port))


// JWT Authenticate
function validateToken(req, res, next) {
  if (environment == "production") { return next() }

  const token = req.headers["authorization"]
  if (!token)
    return res.status(200).send({ code: "400", status: "Access denied, no authorization token received" });

  jwt.verify(token, process.env.SECRET, (err, user) => {
    if (err)
      return res.status(200).send({ code: "400", status: "Access denied, token expired or incorrect" });
    next()
  })
}

// DataBase connection
if (environment != "testing") {
  let timeOut = setInterval(() => {
    mongoose.connect(dbLink, { useNewUrlParser: true }, (err) => {
      if (err) {
        console.log("Encountered an error in Db Connection")
      } else {
        console.log("Succesfully connected with DB");
        clearInterval(timeOut)
      }
    })
  }, 5000);
}

// +++++++++++++++ GRAPHQL METHODS +++++++++++++++++ //
const { graphqlHTTP } = require('express-graphql');
const schema = require("./graphql/schema")
const resolvers = require("./graphql/resolvers")

app.use('/graphql', graphqlHTTP({
  schema: schema,
  rootValue: resolvers,
  graphiql: true,
}));

// ++++++++++++++++ HTTP METHODS +++++++++++++++++++ //

app.get("/", (req, res) => {
  res.send("E-skeleton-graphql is up and running! :D")
})

app.get("/users", async (req, res) => {					//	 B O R R A R
  const users = await User.find();					    //	 B O R R A R
  res.json(users);									            //	 B O R R A R
});

app.get("/deleteUsers", async (req, res) => {			//	 B O R R A R
  const users = await User.deleteMany();				  //	 B O R R A R
  res.json("Users deleted");							        //	 B O R R A R
});



module.exports = app