<script lang="ts">
  type Props = {
    dateString: string;
  };

  const MONTHS = [
    "Jan",
    "Feb",
    "Mar",
    "Apr",
    "May",
    "Jun",
    "Jul",
    "Aug",
    "Sep",
    "Oct",
    "Nov",
    "Dec",
  ];

  const DAY_MS = 24 * 60 * 60 * 1000;

  let { dateString }: Props = $props();

  const pad = (number: number): string =>
    `${number.toString().padStart(2, "0")}`;

  const format = (dateString: string): string => {
    const date = new Date(dateString);
    const now = new Date();
    if (now.valueOf() - date.valueOf() > DAY_MS) {
      return `${date.getDate()} ${MONTHS[date.getMonth()]}`;
    }

    return `${pad(date.getHours())}:${pad(date.getMinutes())}`;
  };
</script>

<time datetime={dateString}>{format(dateString)}</time>
