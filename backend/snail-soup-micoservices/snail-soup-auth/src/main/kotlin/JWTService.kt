package org.snailsoup.auth

import com.auth0.jwt.JWT
import com.auth0.jwt.algorithms.Algorithm
import org.springframework.stereotype.Service
import java.util.UUID

@Service
class JWTService {
    fun readUserFromToken(token: String): AppUser {
        val algorithm = Algorithm.HMAC256("my_ultra_secure_secret")
        val processedToken = JWT.require(algorithm).withClaimPresence("id")
            .withClaimPresence("created_at")
            .withClaimPresence("exp")
            .build()
            .verify(token)

        return AppUser(UUID.fromString(processedToken.getClaim("id").asString()))
    }
}

