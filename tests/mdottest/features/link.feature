Feature: Managing GraphLinks.

    Scenario: Empty GraphLink
        When we build a new GraphLink named "test"
        Then the GraphLink is named "test"
        And the GraphLink has 0 attributes
        And the GraphLink has 0 known entities

    Scenario Outline: Adding attribute to GraphLink
        Given a new GraphLink named "<name>"

        Examples:
            |name|
