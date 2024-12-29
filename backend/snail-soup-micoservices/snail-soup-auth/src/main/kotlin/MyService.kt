package org.snailsoup

import org.springframework.boot.context.properties.EnableConfigurationProperties;
import org.springframework.stereotype.Service;
import org.springframework.web.bind.annotation.GetMapping

@Service
class MyService {
    @GetMapping
    fun Massage(): String {
        return "DupaDupa"
    }
}