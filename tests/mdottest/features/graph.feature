Feature: Handling full graphs

    Scenario: Empty Graph
        When we build a new gaph named "test"
        Then the graph is named "test"
        And the graph has 0 entities
        And the graph has 0 links
