Feature: Register user

    Background:
        * url baseUrl

    Scenario: Successful registration
        Given path 'auth/register'
        And request
        """
        {
            "email": "john.doe@gmail.com",
            "username": "johndoe",
            "password": "V3ryS3cUreP@ssw0rd!"
        }
        """
        When method post
        Then status 201
        Then match response.userId == "#uuid"

    Scenario: Email already taken
        Given path 'auth/register'
        And request
        """
        {
            "email": "john.doe@gmail.com",
            "username": "maxverstappen",
            "password": "V3ryS3cUreP@ssw0rd!"
        }
        """
        When method post
        Then status 409
        Then match response.detail == "email already taken"

    Scenario: Username already taken
        Given path 'auth/register'
        And request
        """
        {
            "email": "max.verstappen@gmail.com",
            "username": "johndoe",
            "password": "V3ryS3cUreP@ssw0rd!"
        }
        """
        When method post
        Then status 409
        Then match response.detail == "username already taken"
