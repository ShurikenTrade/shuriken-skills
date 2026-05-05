/**
 * Shuriken skills plugin for OpenCode.ai
 *
 * Registers the bundled `skills/` directory with OpenCode's skill loader so
 * agents can pull Shuriken integration guidance via the native `skill` tool.
 *
 * Skills are pull-on-demand — there is no global bootstrap injection.
 */

import path from 'path';
import { fileURLToPath } from 'url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const skillsDir = path.resolve(__dirname, '../../skills');

export const ShurikenSkillsPlugin = async () => {
  return {
    config: async (config) => {
      config.skills = config.skills || {};
      config.skills.paths = config.skills.paths || [];
      if (!config.skills.paths.includes(skillsDir)) {
        config.skills.paths.push(skillsDir);
      }
    }
  };
};
