<script lang="ts">
  type Props = {
    dateString: string;
  };

  let { dateString }: Props = $props();

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

  const pad = (number: number): string =>
    `${number.toString().padStart(2, "0")}`;

  const format = (dateString: string): string => {
    const date = new Date(dateString);
    const now = new Date();
    if (
      date.getDate() === now.getDate() ||
      date.getMonth() === now.getMonth() ||
      date.getFullYear() === now.getFullYear()
    ) {
      return `${pad(date.getHours())}:${pad(date.getMinutes())}`;
    }
    return `${date.getDate()} ${MONTHS[date.getMonth()]}`;
  };
</script>

<time datetime={dateString}>{format(dateString)}</time>
