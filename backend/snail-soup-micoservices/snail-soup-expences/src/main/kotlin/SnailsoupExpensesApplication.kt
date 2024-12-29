package org.snailsoup.expences

import org.springframework.boot.autoconfigure.SpringBootApplication
import org.springframework.boot.runApplication
import org.springframework.web.bind.annotation.GetMapping


@SpringBootApplication(scanBasePackages = ["org.snailsoup"])
class SnailsoupExpensesApplication


fun main(args: Array<String>) {
    runApplication<SnailsoupExpensesApplication>(*args)
}