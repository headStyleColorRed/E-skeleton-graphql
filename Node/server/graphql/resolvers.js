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
  },

  createUser: async args => {
    try {
      const { email } = args.user
      const user = new User({
        email: email
      })

      const newUser = await user.save()
      return { ... newUser._doc, _id: newUser.id }
    } catch (error) {
      throw error
    }
  }
}