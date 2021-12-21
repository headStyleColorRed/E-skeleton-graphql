// User.model.js
const mongoose = require("mongoose");

const userSchema = new mongoose.Schema({
	email: {
        type: String,
        required: [true, "can't be blank"],
        unique: true,
    },
	logged: {
		type: Boolean,
		default: false
	}
	
});

const User = mongoose.model("User", userSchema);

module.exports = User;