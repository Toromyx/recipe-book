/*
 * @see https://github.com/tc39/proposal-intl-messageformat
 */

import { MessageFormat } from "messageformat";
// @ts-expect-error apparently it is not possible to extend a global namespace
Intl.MessageFormat = MessageFormat;
