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
        And an entity named "ent1" in graph
        And an entity named "ent2" in graph
        When we link "ent1" and "ent2" under the name "ltest"
        Then the graph has a GraphLink named "ltest"
        And the GraphLink "ltest" knows an entity named "ent1"
        And the GraphLink "ltest" knows an entity named "ent2"

    Scenario: Ternaries
        Given a new graph
        And an entity named "ent1" in graph
        And an entity named "ent2" in graph
        And an entity named "ent3" in graph
        When we link "ent1" and "ent2" under the name "ltest"
        And we add "ent3" to the GraphLink "ltest"
        Then the graph has a GraphLink named "ltest"
        And the GraphLink "ltest" knows an entity named "ent1"
        And the GraphLink "ltest" knows an entity named "ent2"
        And the GraphLink "ltest" knows an entity named "ent3"
