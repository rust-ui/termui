FROM node:24-bookworm-slim AS builder
ENV PNPM_HOME=/pnpm
ENV PATH=$PNPM_HOME:$PATH
WORKDIR /app
RUN corepack enable
COPY . .
ENV NEXT_TELEMETRY_DISABLED=1
ENV SITE_URL=https://termui.rustify.app
RUN --mount=type=cache,id=termui-pnpm-store,target=/pnpm/store \
    pnpm install --frozen-lockfile \
    && pnpm build:production

FROM node:24-bookworm-slim AS runner
WORKDIR /app
ENV NODE_ENV=production \
    NEXT_TELEMETRY_DISABLED=1 \
    HOSTNAME=0.0.0.0 \
    PORT=3000
RUN groupadd --system --gid 1001 termui \
    && useradd --system --uid 1001 --gid termui --home-dir /app --shell /usr/sbin/nologin termui
COPY --from=builder --chown=termui:termui /app/public ./public
COPY --from=builder --chown=termui:termui /app/.next/standalone ./
COPY --from=builder --chown=termui:termui /app/.next/static ./.next/static
USER termui
EXPOSE 3000
CMD ["node", "server.js"]
