import { defineCollection, z } from 'astro:content';
import { docsLoader } from '@astrojs/starlight/loaders';
import { docsSchema } from '@astrojs/starlight/schema';

export const collections = {
  docs: defineCollection({
    loader: docsLoader(),
    schema: docsSchema({
      extend: z.object({
        /** LeetCode problem number (e.g. 1 for Two Sum) */
        leetcodeNumber: z.number().optional(),
        /** Problem difficulty level */
        difficulty: z.enum(['Easy', 'Medium', 'Hard']).optional(),
        /** LeetCode URL slug (e.g. "two-sum") */
        leetcodeSlug: z.string().optional(),
        /** Topic/category tags */
        tags: z.array(z.string()).optional(),
      }),
    }),
  }),
};
