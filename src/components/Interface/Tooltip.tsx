import { Transition } from "@headlessui/react";
import { Fragment, useEffect, useRef, useState } from "react";
type Pos = {
	top?: number;
	left?: number;
	bottom?: number;
	right?: number;
};
export const Tooltip = ({
	content,
	width,
	...props
}: {
	content?: React.ReactNode;
	width?: number;
} & React.HTMLAttributes<HTMLDivElement>) => {
	const [show, setShow] = useState(false);
	const [tooltipPos, setTooltipPos] = useState<Pos | null>(null);
	const ref = useRef<HTMLDivElement>(null);
	const handleMouseEnter = (event: React.MouseEvent<HTMLDivElement>) => {
		const parentRect = ref.current?.getBoundingClientRect();
		const windowWidth = window.innerWidth;
		const windowHeight = window.innerHeight;
		ref.current?.addEventListener("mousemove", (e) => {
			const pos: Pos = {};
			if (parentRect) {
				if (parentRect.left > windowWidth / 2) {
					//右側にある
					pos.right = windowWidth - e.clientX;
				} else {
					pos.left = e.clientX;
				}
				if (parentRect.top > windowHeight / 2) {
					//下側にある
					pos.bottom = windowHeight - e.clientY; //-(ref.current?.clientHeight || 0);
				} else {
					pos.top = e.clientY;
				}
			}
			setTooltipPos(pos);
		});
		setShow(true);
	};

	const handleMouseLeave = () => {
		setShow(false);
		ref.current?.removeEventListener("mousemove", () => {});
	};

	return (
		<>
			<div
				{...props}
				ref={ref}
				onMouseEnter={handleMouseEnter}
				onMouseLeave={handleMouseLeave}
			>
				{props.children}
			</div>
			<Transition
				appear
				show={show}
				as={Fragment}
				enter="ease-out duration-75"
				enterFrom="opacity-0 scale-95"
				enterTo="opacity-100 scale-100"
				leave="ease-in duration-75"
				leaveFrom="opacity-100 scale-100"
				leaveTo="opacity-0 scale-95"
			>
				<div
					style={{
						position: "fixed",
						...tooltipPos,
						maxWidth: 500,
						width,
					}}
					className="bg-white text-[16px] p-2 text-[black] whitespace-pre-wrap flex-none mx-3 my-1 rounded-md border-[3px] font-sans z-50"
				>
					{content}
				</div>
			</Transition>
		</>
	);
};
