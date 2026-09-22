"use client";

import Image from "next/image";
import Link from "next/link";

/**
 * Client card for the /authors index. Visuals: avatar · name · role · article
 * count, matching the RUSTIFY editorial card pattern without analytics.
 */

const CARD_SHADOW = "border border-black/[0.05] shadow-md";

export function AuthorCard({
  slug,
  name,
  role,
  avatar,
  articleCount,
}: {
  slug: string;
  name: string;
  role: string;
  avatar: string;
  articleCount: number;
}) {
  return (
    <Link
      href={`/authors/${slug}`}
      className={`group flex flex-col gap-4 rounded-[24px] bg-white p-6 transition-transform duration-200 hover:-translate-y-0.5 ${CARD_SHADOW}`}
    >
      <div className="size-[64px] overflow-hidden rounded-[16px] bg-black/[0.03] ring-2 ring-white">
        <Image
          src={avatar}
          alt={name}
          width={64}
          height={64}
          className="size-full object-cover"
        />
      </div>
      <div className="flex flex-col gap-1">
        <h2 className="text-[20px] font-semibold tracking-[-0.03em] text-[#0f0f10]">
          {name}
        </h2>
        <p className="text-[14px] text-[rgba(15,15,16,0.73)]">{role}</p>
      </div>
      <span className="mt-auto text-[13px] tabular-nums text-[rgba(15,15,16,0.5)]">
        {articleCount} articles
      </span>
    </Link>
  );
}
