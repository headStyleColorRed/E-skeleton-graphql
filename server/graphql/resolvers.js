const User = require("../mongoDB/userModel.js")

module.exports = {
  users: async () => {
    try {
      const fetchedUsers = await User.find()
      return fetchedUsers.map(user => {
        return {
          ...user._doc,
          _id: user.id,
        }
      })
    } catch (error) {
      throw error
    }
  }
}