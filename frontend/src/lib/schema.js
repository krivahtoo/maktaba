import { z } from 'zod';

export const loginSchema = z.object({
  username: z
    .string()
    .min(2, {
      message: 'Username too short'
    })
    .max(50, {
      message: 'Username too long'
    }),
  password: z
    .string()
    .min(6, {
      message: 'Password too short'
    })
    .max(100, {
      message: 'Password too long'
    })
});

export const registerSchema = z
  .object({
    name: z
      .string()
      .min(2, {
        message: 'Name must contain at least 2 characters'
      })
      .max(255),
    email: z
      .string()
      .email({ message: 'Please provide a valid email address.' })
      .min(2, {
        message: 'Name must contain at least 2 characters'
      })
      .max(255),
    photo: z
      .instanceof(File, { message: 'Please upload a file.' })
      .refine((f) => f.size < 2_000_000, 'Max 2 MB upload size.')
      .refine(
        (f) => f.type === 'image/jpeg' || f.type === 'image/png',
        'Only JPEG or PNG files are allowed.'
      ),
    username: z
      .string()
      .min(2, {
        message: 'Username must contain at least 2 characters'
      })
      .max(50, {
        message: 'Username must contain less than 50 characters'
      }),
    password: z
      .string()
      .min(6, {
        message: 'Password must contain at least 6 characters'
      })
      .max(100),
    confirm: z.string()
  })
  .refine((o) => o.password === o.confirm, {
    message: 'Passwords do not match',
    path: ['confirm']
  });

/** @typedef {typeof registerSchema} RegisterSchema */

/** @typedef {typeof loginSchema} LoginSchema */
