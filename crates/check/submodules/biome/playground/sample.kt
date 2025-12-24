package com.example.demo

import java.util.*

// Sample Kotlin file with intentional formatting issues
class Person(
	val name: String,
	val age: Int,
) {
	var email: String? = null

	fun greet() {
		println("Hello, my name is $name and I am $age years old")
		if (email != null) {
			println("Email: $email")
		}
	}

	fun isAdult() = age >= 18
}

data class Address(
	val street: String,
	val city: String,
	val zipCode: String,
)

fun main(args: Array<String>) {
	val person = Person("Alice", 30)
	person.email = "alice@example.com"

	person.greet()

	val address = Address("123 Main St", "Springfield", "12345")
	println("Address: ${address.street}, ${address.city}, ${address.zipCode}")

	val numbers = listOf(1, 2, 3, 4, 5)
	val doubled = numbers.map { it * 2 }
	println("Doubled: $doubled")

	// Unused variable
	val unused = "This is not used"

	// Long line that should be wrapped
	val longString = "This is a very long string that exceeds the recommended line length and should ideally be wrapped or broken into multiple lines for better readability"
}
