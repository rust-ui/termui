import { z } from "zod";

const eventSchema = z.object({
  name: z.enum([
    "copy_npm_command",
    "copy_usage_code",
    "copy_primitive_code",
    "keyboard_shortcut_navigate",
    "click_edit_page",
    "open_command_menu",
    "click_registry_add_button",
    "copy_registry_command",
  ]),
  properties: z
    .record(
      z.string(),
      z.union([z.string(), z.number(), z.boolean(), z.null()])
    )
    .optional(),
});

export type Event = z.infer<typeof eventSchema>;

// Keep event payloads validated without sending user activity to a hosting vendor.
export const trackEvent = (input: Event): void => {
  eventSchema.parse(input);
};
