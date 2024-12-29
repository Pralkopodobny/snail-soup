package org.snailsoup.expences

import org.snailsoup.auth.MyService
import org.springframework.web.bind.annotation.GetMapping
import org.springframework.web.bind.annotation.RequestMapping
import org.springframework.web.bind.annotation.RestController

@RestController
@RequestMapping("demo")
class DemoController(val service: MyService) {

    @GetMapping()
    fun Hello(): String {
        return service.Massage()
    }
}