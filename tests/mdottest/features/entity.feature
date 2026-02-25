Feature: Managing individual entities

    Scenario: Empty entity
        When we build a new entity named "test"
        Then the entity is named "test"
