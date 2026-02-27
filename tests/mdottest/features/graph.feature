Feature: Handling full graphs

    Scenario: Empty Graph
        When we build a new graph
        Then the graph has 0 entities
        And the graph has 0 links

    Scenario: Adding entities
        Given a new graph
        And a new entity named "entest"
        When we add the entity to our graph
        Then the graph has an entity named "entest"

    Scenario: Adding links
        Given a new graph
        And a new GraphLink named "ltest"
        When we add the GraphLink to our graph
        Then the graph has a GraphLink named "ltest"
