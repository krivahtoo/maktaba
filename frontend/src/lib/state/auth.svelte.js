export class Auth {
  /** @type {string | undefined} */
  token = $state();

  user = $state();

  reset() {
    this.token = '';
    this.user = undefined;
  }
}

export const auth = new Auth();
