Feature: Handling full graphs

    Scenario: Empty Graph
        When we build a new graph
        Then the graph has 0 entities
        And the graph has 0 links
