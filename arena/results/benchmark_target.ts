
export interface User {
  id: string;
  name: string;
  roles: string[];
}
export function processUser(user: User): string {
  return `User: ${user.name} (${user.roles.join(', ')})`;
}
export const users: User[] = Array.from({ length: 1000 }, (_, i) => ({
  id: `id_${i}`,
  name: `User ${i}`,
  roles: ['admin', 'developer']
}));
