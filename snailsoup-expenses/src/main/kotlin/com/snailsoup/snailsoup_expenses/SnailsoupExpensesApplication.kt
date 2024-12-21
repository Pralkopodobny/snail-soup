package com.snailsoup.snailsoup_expenses

import org.springframework.boot.autoconfigure.SpringBootApplication
import org.springframework.boot.runApplication
import org.springframework.web.bind.annotation.GetMapping


@SpringBootApplication
class SnailsoupExpensesApplication


fun main(args: Array<String>) {
	runApplication<SnailsoupExpensesApplication>(*args)
}
