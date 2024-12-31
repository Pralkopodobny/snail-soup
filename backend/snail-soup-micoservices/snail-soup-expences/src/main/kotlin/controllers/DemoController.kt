package org.snailsoup.expences.controllers

import org.snailsoup.auth.MyService
import org.springframework.web.bind.annotation.GetMapping
import org.springframework.web.bind.annotation.RequestMapping
import org.springframework.web.bind.annotation.RestController

@RestController
@RequestMapping("demo")
class DemoController(val service: MyService) {

    @GetMapping("hello")
    fun hello(): String {
        return service.Massage()
    }

    @GetMapping("age")
    fun hello2(age : Int): Int {
        return age + 10;
    }

    @GetMapping("age-object")
    fun hello3(age : Int): DemoAge {
        return DemoAge(age + 10)
    }

    class DemoAge(val age: Int)
}