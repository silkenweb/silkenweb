/// <reference types="cypress" />

declare namespace Cypress {
  interface Chainable<Subject> {
    /**
     * Create several Todo items via UI
     * @example
     * cy.createDefaultTodos()
     */
    createDefaultTodos(): Chainable<any>
    /**
     * Creates one Todo using UI
     * @example
     * cy.createTodo('new item')
     */
    createTodo(title: string): Chainable<any>

    /**
     * Command that injects Axe core library into app html.
     * @example
     *  cy.visit('/')
     *  cy.v()
     */
    addAxeCode(): Chainable<any>

    /**
     * Run a11y tests or only a subset of all tests
     * @see https://github.com/avanslaars/cypress-axe
     * @example
     *  cy.checkA11y()
     */
    checkA11y(any, object): Chainable<any>
  }
}
